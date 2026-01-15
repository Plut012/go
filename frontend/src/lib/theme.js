let currentTheme = null;

/**
 * Load a theme by name
 */
export async function loadTheme(themeName) {
  try {
    // Fetch theme manifest
    const manifestUrl = `/themes/${themeName}/manifest.json`;
    const response = await fetch(manifestUrl);

    if (!response.ok) {
      throw new Error(`Failed to load theme: ${themeName}`);
    }

    currentTheme = await response.json();
    currentTheme.name = themeName;

    // Load theme CSS
    const existingLink = document.getElementById('theme-css');
    if (existingLink) {
      existingLink.remove();
    }

    const link = document.createElement('link');
    link.id = 'theme-css';
    link.rel = 'stylesheet';
    link.href = `/themes/${themeName}/theme.css`;
    document.head.appendChild(link);

    console.log(`Theme loaded: ${themeName}`);
    return currentTheme;
  } catch (error) {
    console.error('Failed to load theme:', error);
    throw error;
  }
}

/**
 * Get URL for a piece SVG
 */
export function getPieceUrl(color, groupSize = 1) {
  if (!currentTheme) {
    console.warn('No theme loaded');
    return '';
  }

  const themeName = currentTheme.name;
  const pieces = currentTheme.pieces[color];

  // Simple theme: just one piece per color
  if (typeof pieces === 'string') {
    return `/themes/${themeName}/${pieces}`;
  }

  // Complex theme: pieces based on group size
  // TODO: Implement size-based piece selection for grim-toon theme
  return `/themes/${themeName}/${pieces}`;
}

/**
 * Get current theme name
 */
export function getCurrentTheme() {
  return currentTheme?.name || null;
}
