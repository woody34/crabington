module.exports = {
    apps: [
        {
                name: "TailwindCSS",
                script: "npm run tw:dev",
                ignore_watch: ["."],
                env: { NODE_ENV: "development" },
        },
        {
                name: "Yew",
                script: "npm run start",
                ignore_watch: ["."],
                env: { NODE_ENV: "development" },
        },
    ],
};