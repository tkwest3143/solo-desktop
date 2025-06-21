import { describe, test, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ResizeWindowButton from '~/components/ResizeWindowButton.vue'

// Mock tauri API
const mockSetSize = vi.fn()
const mockSetPosition = vi.fn()
const mockGetCurrentWindow = vi.fn(() => ({
  setSize: mockSetSize,
  setPosition: mockSetPosition,
  outerSize: vi.fn().mockResolvedValue({ width: 1920, height: 1080 }),
  innerSize: vi.fn().mockResolvedValue({ width: 1920, height: 1080 })
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: mockGetCurrentWindow,
  LogicalPosition: class {
    constructor(public x: number, public y: number) {}
  },
  LogicalSize: class {
    constructor(public width: number, public height: number) {}
  }
}))

// Mock vue-router
const mockPush = vi.fn()
vi.mock('vue-router', () => ({
  useRoute: () => ({ query: {} }),
  useRouter: () => ({ push: mockPush })
}))

describe('ResizeWindowButton', () => {
  test('displays correct text for normal mode', () => {
    const wrapper = mount(ResizeWindowButton)
    expect(wrapper.text()).toContain('小さく表示')
  })

  test('window mode toggles between normal and compact', async () => {
    const wrapper = mount(ResizeWindowButton)
    
    // Initially should be normal mode
    expect(wrapper.text()).toContain('小さく表示')
    
    // Click to toggle to compact mode
    await wrapper.find('button').trigger('click')
    
    // Wait for next tick to allow async operations
    await wrapper.vm.$nextTick()
    
    // Check if text changed to "大きく表示"
    expect(wrapper.text()).toContain('大きく表示')
  })
})