/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	darkMode: "class",
	plugins: [require("@tailwindcss/forms")],
};
