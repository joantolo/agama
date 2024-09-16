module.export = [
  {
    root: true,
    env: {
      browser: true,
      es6: true,
      jest: true,
    },
    extends: [
      "eslint:recommended",
      "standard",
      "standard-jsx",
      "standard-react",
      "plugin:@typescript-eslint/recommended",
      "prettier",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
      ecmaVersion: 7,
      ecmaFeatures: {
        jsx: true,
      },
      sourceType: "module",
    },
    plugins: ["agama-i18n", "flowtype", "i18next", "react", "react-hooks", "@typescript-eslint"],
    rules: {
      "agama-i18n/string-literals": "error",
      "i18next/no-literal-string": "error",
      "no-var": "error",
      "no-multi-str": "off",
      "no-use-before-define": "off",
      "@typescript-eslint/no-unused-vars": "off",
      "@typescript-eslint/no-use-before-define": "warn",
      "@typescript-eslint/ban-ts-comment": "off",
      "lines-between-class-members": [
        "error",
        "always",
        {
          exceptAfterSingleLine: true,
        },
      ],
      "prefer-promise-reject-errors": [
        "error",
        {
          allowEmptyReject: true,
        },
      ],
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "error",
      camelcase: "off",
      "comma-dangle": "off",
      curly: "off",
      "jsx-quotes": "off",
      "key-spacing": "off",
      "no-console": "off",
      quotes: "off",
      "react/jsx-curly-spacing": "off",
      "react/jsx-indent-props": "off",
      "react/prop-types": "off",
      "space-before-function-paren": "off",
      "n/no-callback-literal": "off",
    },
    overrides: [
      {
        // do not check translations in the testing or development files
        files: ["*.test.*", "test-utils.js"],
        rules: {
          "i18next/no-literal-string": "off",
        },
      },
      {
        // do not check translation arguments in the test, it checks some internals by passing variables
        files: ["i18n.test.js"],
        rules: {
          "agama-i18n/string-literals": "off",
        },
      },
    ],
    globals: {
      require: false,
      module: false,
    },
    ignores: ["node_modules/*", "src/lib/*", "src/**/test-data/*"],
  },
];
