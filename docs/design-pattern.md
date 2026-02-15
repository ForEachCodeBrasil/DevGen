# Visual Design Pattern - DevGen

## Overview

DevGen uses a modern, developer-focused dark theme with cyberpunk and technical aesthetics. This design is optimized for high information density, rapid interaction, and a "terminal-like" feel that resonates with software developers.

## Color Palette

The application follows a "Deep Space" color scheme, prioritizing high contrast and neon accents.

### Background Colors

| Name             | Hex       | Usage                                |
| ---------------- | --------- | ------------------------------------ |
| Deep Space Black | `#0d1117` | Primary background                   |
| Dark Surface     | `#161b22` | Cards, modals, elevated surfaces     |
| Border Dark      | `#21262d` | Standard borders                     |
| Border Highlight | `#30363d` | Hover states, active borders         |

### Text Colors

| Name           | Class           | Usage                         |
| -------------- | --------------- | ----------------------------- |
| Primary Text   | `text-gray-100` | Headings, primary content     |
| Secondary Text | `text-gray-400` | Descriptions, helper text     |
| Muted Text     | `text-gray-500` | Placeholders, disabled states |

### Accent Colors

| Name             | Class              | Usage                                 |
| ---------------- | ------------------ | ------------------------------------- |
| Neon Green       | `text-neon-green`  | Primary actions, success states, CTAs |
| Neon Green Light | `text-neon-green-light` | Hover states, highlights         |
| Error Red        | `text-error-red`   | Errors, destructive actions           |
| Warning Amber    | `text-warning-amber` | Warnings, alerts                    |
| Info Blue        | `text-info-blue` | Information, links                      |

## Typography

### Font Family

```css
font-family: 'JetBrains Mono', monospace;
```

**Why JetBrains Mono?**

- Dedicated developer aesthetic.
- Excellent readability for numeric and mock data.
- Consistent character width for easier data parsing.

### Font Sizes

- **Page Title**: `text-2xl` (24px) - Used in main headers.
- **Section Title**: `text-xl` (20px) - Used for category headers.
- **Card Title**: `text-sm` (14px) - Used for generator names.
- **Body Text**: `text-xs` (12px) - Used for descriptions and labels.
- **Technical Text**: `text-[10px]` - Used for metadata and status codes.

## UI Components

### Background Grid

A subtle 40px grid pattern is applied to the root level to reinforce the "technical blueprint" aesthetic.

```css
background-image: 
  linear-gradient(to right, #21262d 1px, transparent 1px),
  linear-gradient(to bottom, #21262d 1px, transparent 1px);
background-size: 40px 40px;
```

### Borders & Corners

- **Radius**: Use sharp `rounded-sm` (2px) or `rounded-none` for all components.
- **Glow**: Use subtle neon shadows for primary actions.
  - `shadow-neon`: `0 0 15px rgba(34, 197, 94, 0.3)`

### Layout Patterns

- **Cards**: `.card` class provides a `Dark Surface` background with a `Border Dark` edge.
- **Buttons**:
  - `.btn-primary`: Neon Green background with black text.
  - `.btn-secondary`: Transparent background with gray borders.
  - `.btn-danger`: Error Red background with white text.
- **Inputs**: `.input-field` class uses `Deep Space Black` for a recessed feel.

## Animations

- **Transitions**: All hover states should use `transition-all duration-200`.
- **Glow Pulse**: Important status indicators should use the `animate-glow` animation.

## Interaction Model

1. **Focus First**: The application is designed to be usable primarily via keyboard.
2. **Speed**: Generators should produce results instantly on click.
3. **Tray Synergy**: The app is built to be a utility that "disappears" when not in use (Blur-to-Hide).
4. **Copy-on-Click**: Primary results should always be actionable (Copy to clipboard) with a single interaction.
