export const sizes = ['default', 'sm', 'lg'] as const;
export type Size = typeof sizes[number];

export const colors = ['default', 'primary', 'secondary', 'alternative', 'success', 'error'] as const;
export type Color = typeof colors[number];
