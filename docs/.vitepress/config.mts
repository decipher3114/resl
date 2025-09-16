import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "RESL",
    description:
        "Runtime Evaluated Serialization Language - A modern configuration and serialization language with variables, expressions, and dynamic runtime evaluation",
    base: "/resl/",
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [{ text: "Home", link: "/" }],

        sidebar: [
            {
                text: "Introduction",
                items: [
                    { text: "Getting Started", link: "/introduction/getting-started" },
                    { text: "What is RESL?", link: "/introduction/what-is-resl" },
                    { text: "Why RESL?", link: "/introduction/why-resl" },
                ],
            },
            {
                text: "Syntax Guide",
                items: [
                    { text: "Overview", link: "/syntax-guide/overview" },
                    { text: "Literals", link: "/syntax-guide/literals" },
                    { text: "Blocks", link: "/syntax-guide/blocks" },
                    { text: "Collections", link: "/syntax-guide/collections" },
                    { text: "Operations", link: "/syntax-guide/operations" },
                    { text: "Functions", link: "/syntax-guide/functions" },
                    { text: "Control Flow", link: "/syntax-guide/control-flow" },
                    { text: "Best Practices", link: "/syntax-guide/best-practices" },
                ],
            },
            {
                text: "Usage",
                items: [
                    { text: "CLI Usage", link: "/usage/cli-usage" },
                    { text: "Language Bindings", link: "/usage/bindings" },
                ],
            },
        ],

        socialLinks: [
            { icon: "github", link: "https://github.com/decipher3114/resl" },
        ],

        search: {
            provider: "local",
        },
    },
    markdown: {
        lineNumbers: true,
    },
});
