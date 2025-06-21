import { colorOptions, defaultColor, semanticColors, type ColorOption } from '~/assets/colors';

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

  // Get semantic color based on current theme
  const getSemanticColor = (category: keyof typeof semanticColors, subcategory: string, property?: string): string => {
    const isDark = colorMode.value === 'dark';
    const colorCategory = semanticColors[category] as any;
    
    if (!colorCategory || !colorCategory[subcategory]) return '';
    
    const colorSet = colorCategory[subcategory];
    
    if (property && colorSet[property]) {
      return isDark ? colorSet[property].dark : colorSet[property].light;
    }
    
    return isDark ? colorSet.dark : colorSet.light;
  };

  // Get background color classes
  const getBgClass = (type: 'primary' | 'secondary' | 'tertiary' | 'hover' | 'active' = 'primary'): string => {
    const isDark = colorMode.value === 'dark';
    const colors = semanticColors.background[type];
    return isDark ? 'dark:bg-slate-800' : 'bg-white';
  };

  // Get text color classes
  const getTextClass = (type: 'primary' | 'secondary' | 'tertiary' | 'inverse' = 'primary'): string => {
    const isDark = colorMode.value === 'dark';
    
    switch (type) {
      case 'primary':
        return isDark ? 'dark:text-slate-50' : 'text-slate-900';
      case 'secondary':
        return isDark ? 'dark:text-slate-300' : 'text-slate-600';
      case 'tertiary':
        return isDark ? 'dark:text-slate-500' : 'text-slate-400';
      case 'inverse':
        return isDark ? 'dark:text-slate-900' : 'text-white';
      default:
        return isDark ? 'dark:text-slate-50' : 'text-slate-900';
    }
  };

  // Get border color classes
  const getBorderClass = (type: 'default' | 'focus' | 'active' = 'default'): string => {
    const isDark = colorMode.value === 'dark';
    
    switch (type) {
      case 'default':
        return isDark ? 'dark:border-slate-600' : 'border-slate-200';
      case 'focus':
      case 'active':
        return isDark ? 'dark:border-blue-400' : 'border-blue-500';
      default:
        return isDark ? 'dark:border-slate-600' : 'border-slate-200';
    }
  };

  // Get state color classes
  const getStateClasses = (state: 'error' | 'success' | 'warning' | 'info', type: 'bg' | 'text' | 'border' | 'hover' = 'bg'): string => {
    const isDark = colorMode.value === 'dark';
    const stateColors = semanticColors.state[state];
    
    if (!stateColors || !stateColors[type]) return '';
    
    switch (state) {
      case 'error':
        switch (type) {
          case 'bg': return isDark ? 'dark:bg-red-900' : 'bg-red-50';
          case 'text': return isDark ? 'dark:text-red-300' : 'text-red-600';
          case 'border': return isDark ? 'dark:border-red-600' : 'border-red-300';
          case 'hover': return isDark ? 'dark:hover:bg-red-800' : 'hover:bg-red-100';
        }
        break;
      case 'success':
        switch (type) {
          case 'bg': return isDark ? 'dark:bg-green-900' : 'bg-green-50';
          case 'text': return isDark ? 'dark:text-green-300' : 'text-green-600';
          case 'border': return isDark ? 'dark:border-green-600' : 'border-green-300';
          case 'hover': return isDark ? 'dark:hover:bg-green-800' : 'hover:bg-green-100';
        }
        break;
      case 'warning':
        switch (type) {
          case 'bg': return isDark ? 'dark:bg-amber-900' : 'bg-amber-50';
          case 'text': return isDark ? 'dark:text-amber-300' : 'text-amber-600';
          case 'border': return isDark ? 'dark:border-amber-600' : 'border-amber-300';
          case 'hover': return isDark ? 'dark:hover:bg-amber-800' : 'hover:bg-amber-100';
        }
        break;
      case 'info':
        switch (type) {
          case 'bg': return isDark ? 'dark:bg-blue-900' : 'bg-blue-50';
          case 'text': return isDark ? 'dark:text-blue-300' : 'text-blue-600';
          case 'border': return isDark ? 'dark:border-blue-600' : 'border-blue-300';
          case 'hover': return isDark ? 'dark:hover:bg-blue-800' : 'hover:bg-blue-100';
        }
        break;
    }
    
    return '';
  };

  // Get priority badge classes based on theme (updated to use semantic colors)
  const getPriorityBadgeClass = (priority?: string): string => {
    switch (priority) {
      case 'high':
        return `${getStateClasses('error', 'bg')} ${getStateClasses('error', 'text')}`;
      case 'low':
        return `${getBgClass('secondary')} ${getTextClass('secondary')}`;
      default:
        return `${getStateClasses('info', 'bg')} ${getStateClasses('info', 'text')}`;
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
    getSemanticColor,
    getBgClass,
    getTextClass,
    getBorderClass,
    getStateClasses,
    getPriorityBadgeClass,
    getPriorityLabel,
    defaultColor,
    semanticColors
  };
};