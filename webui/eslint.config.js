import antfu from '@antfu/eslint-config';
import betterTailwindcss from 'eslint-plugin-better-tailwindcss';

export default antfu(
  {
    formatters: true,
    stylistic: {
      semi: true,
    },
    vue: true,
    rules: {
      'style/quote-props': ['error', 'as-needed'],
    },
  },
  {
    plugins: {
      'better-tailwindcss': betterTailwindcss,
    },
    rules: {
      ...betterTailwindcss.configs.recommended.rules,
    },
    settings: {
      'better-tailwindcss': {
        entryPoint: 'src/style.css',
      },
    },
  },
);
