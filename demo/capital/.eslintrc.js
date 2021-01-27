module.exports = {
  "env": {
    "browser": true,
    "es6": true,
    "node": true,
    "commonjs": true
  },
  "extends": ["eslint:recommended", "plugin:react/recommended"],
  "parserOptions": {
    "ecmaFeatures": {
      "jsx": true
    },
    "ecmaVersion": 2018,
    "sourceType": "module"
  },
  "plugins": [
    "react"
  ],
  "globals": {
    "$": true
  },
  "parser": "babel-eslint",
  "rules": {
    "require-atomic-updates": 0,
    "indent": [
      "error",
      "tab"
    ],
    "linebreak-style": [
      "error",
      "windows"
    ],
    "quotes": [
      "error",
      "single"
    ],
    "semi": [
      "error",
      "always"
    ]
  },
  "settings": {
    "react": {
      "version": "16.7"
    }
  }
};