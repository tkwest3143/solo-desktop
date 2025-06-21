import { colorOptions, defaultColor, type ColorOption } from '~/assets/colors';

export const useTheme = () => {
  const colorMode = useColorMode();

  // Get the appropriate color value based on current theme
  const getThemeColor = (color: string): string => {
    if (!color) return defaultColor;
    
    const colorOption = colorOptions.find(option => 
      option.value === color || option.light === color || option.dark === color
    );
    
    if (!colorOption) return color;
    
    return colorMode.value === 'dark' ? (colorOption.dark || colorOption.value) : (colorOption.light || colorOption.value);
  };

  // Get all color options for selection
  const getColorOptions = (): ColorOption[] => {
    return colorOptions;
  };

  // Get priority badge classes based on theme
  const getPriorityBadgeClass = (priority?: string): string => {
    const isDark = colorMode.value === 'dark';
    
    switch (priority) {
      case 'high':
        return isDark ? 'bg-red-900 text-red-200' : 'bg-red-100 text-red-800';
      case 'low':
        return isDark ? 'bg-gray-700 text-gray-300' : 'bg-gray-100 text-gray-800';
      default:
        return isDark ? 'bg-blue-900 text-blue-200' : 'bg-blue-100 text-blue-800';
    }
  };

  // Get priority label
  const getPriorityLabel = (priority?: string): string => {
    switch (priority) {
      case 'high':
        return '高優先度';
      case 'low':
        return '低優先度';
      default:
        return '通常';
    }
  };

  return {
    colorMode,
    getThemeColor,
    getColorOptions,
    getPriorityBadgeClass,
    getPriorityLabel,
    defaultColor
  };
};