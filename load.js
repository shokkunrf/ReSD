import { get } from "resd";

const element = document.getElementById("file_input");
element.addEventListener("change", (e) => {
    const reader = new FileReader();
    reader.addEventListener("load", (e) => {
        const p = get(new Uint8Array(e.target.result));
        console.log(p);
    });

    const file = element.files[0];
    reader.readAsArrayBuffer(file);
});
