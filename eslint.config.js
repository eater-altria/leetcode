const eslintPluginPrettier = require('eslint-plugin-prettier');
const prettierConfig = require('eslint-config-prettier');
const typescriptEslintPlugin = require('@typescript-eslint/eslint-plugin');
const typescriptParser = require('@typescript-eslint/parser');

module.exports = [
  {
    files: ['**/*.ts', '**/*.tsx'], // 仅应用于 TypeScript 文件
    languageOptions: {
      parser: typescriptParser, // 设置解析器
      parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
      },
    },
    plugins: {
      '@typescript-eslint': typescriptEslintPlugin, // 引入 TypeScript 插件
      prettier: eslintPluginPrettier, // 引入 Prettier 插件
    },
    rules: {
      ...typescriptEslintPlugin.configs.recommended.rules, // 使用推荐的 TypeScript 规则
      ...prettierConfig.rules, // 兼容 Prettier 规则
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      '@typescript-eslint/no-explicit-any': 'warn',
      'prettier/prettier': 'error', // Prettier 错误将作为 ESLint 错误
    },
  },
];
