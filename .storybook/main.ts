import type { StorybookViteConfig } from "@storybook/builder-vite";
import postcss from "postcss";
import * as tailwindcss from "../tailwind.config";
import viteConfig from "../vite.config";

const config: StorybookViteConfig = {
  stories: ["../frontend/src/**/*.stories.mdx", "../frontend/src/**/*.stories.@(js|jsx|ts|tsx)"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
    "storybook-addon-react-router-v6",
    {
      name: "@storybook/addon-postcss",
      options: {
        postcssLoaderOptions: {
          implementation: postcss,
          postcssOptions: {
            plugins: {
              tailwindcss,
              autoprefixer: {},
            },
          },
        },
      },
    },
  ],
  framework: "@storybook/react",
  core: {
    builder: "@storybook/builder-vite",
  },
  features: {
    storyStoreV7: true,
  },
  viteFinal: async config => {
    config.resolve = viteConfig.resolve;
    return config;
  },
};

export default config;
