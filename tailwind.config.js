module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {
      colors: {
        'primary': '#3f51b5',
        'secondary': '#c5cae9',
        'background': '#f4f4f4',
        'text': '#333',
        'footer-bg': '#333',
        'footer-text': '#fff',
        'not-watched': '#e74c3c',
        'watched': '#2ecc71',
        'button-bg': '#3f51b5',
        'button-hover-bg': '#303f9f',
      },
      fontFamily: {
        'sans': ['Helvetica Neue', 'Helvetica', 'Arial', 'sans-serif'],
      },
      boxShadow: {
        'card': '0 2px 8px rgba(0,0,0,0.1)',
        'input-focus': '0 0 5px rgba(63, 81, 181, 0.5)',
      },
      borderRadius: {
        'lg': '8px',
        'full': '9999px',
      },
      spacing: {
        'container': '20px',
        'card-width': '300px',
        'thumbnail-height': '180px',
        'input-padding': '10px',
        'input-width': '300px',
        'button-padding': '5px 10px',
        'footer-padding': '15px',
        'header-padding': '20px',
        'header-margin-top': '10px',
        'search-bar-margin-bottom': '30px',
        'program-card-gap': '20px',
        'program-card-margin-bottom': '15px',
        'program-card-padding': '15px',
        'program-title-margin-bottom': '8px',
        'status-margin-top': '10px',
        'status-label-padding': '5px 10px',
        'broadcast-schedule-margin': '5px 0',
      },
      transitionProperty: {
        'color': 'color',
        'transform': 'transform',
        'box-shadow': 'box-shadow',
        'background-color': 'background-color',
      },
      transitionDuration: {
        '300': '300ms',
      },
      backgroundImage: {
        'original-gradient': 'linear-gradient(90deg, rgba(168, 202, 240, 1), rgba(233, 240, 250, 1))',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
