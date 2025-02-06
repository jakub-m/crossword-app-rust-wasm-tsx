import init, { generate_crossword } from "./pkg/crossword.js";
init().then(() => {
    //console.log(join_rs(["hello", "world"]));
    var layout = generate_crossword(["hello", "world"], "Automatic");
    //console.log(layout[0].word)
    //console.log(layout[1].word)
    console.log(
        layout
    );
    //greet("WebAssembly");
});