import { Resd } from "resd";

const reader = new FileReader();
reader.addEventListener("load", (e) => {
    const p = Resd.new(new Uint8Array(e.target.result));
    console.log(p);
});

const element = document.getElementById("file_input");
element.addEventListener("change", (e) => {
    const file = element.files[0];
    reader.readAsArrayBuffer(file);
});
