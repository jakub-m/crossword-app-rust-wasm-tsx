import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
(globalThis as any).FinalizationRegistry = undefined; // No idea why FinalizationRegistry does not work when I run `npm start`, this is a workaround.
// eslint-disable-next-line import/first
import init_crossword_wasm from './crossword_wasm/crossword'

//const root = ReactDOM.createRoot(
//  document.getElementById('root') as HTMLElement
//);

//// If you want to start measuring performance in your app, pass a function
//// to log results (for example: reportWebVitals(console.log))
//// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
//reportWebVitals();

init_crossword_wasm().then(() => {
  const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
  );

  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
});