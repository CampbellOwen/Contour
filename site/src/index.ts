interface IsolinePath {
  class: string;
  path: string;
  fill: string;
}

interface IsolineSVG {
  paths: [IsolinePath];
  view_box: string;
}

function isoline_to_svg(isoline: IsolineSVG): SVGSVGElement {
  let svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("viewBox", isoline.view_box);
  svg.setAttribute("width", "100%");
  svg.setAttribute("height", "100%");
  isoline.paths.forEach((path) => {
    let svg_path = document.createElementNS(
      "http://www.w3.org/2000/svg",
      "path"
    );
    svg_path.setAttribute("d", path.path);
    svg_path.setAttribute("fill", path.fill);
    svg_path.setAttribute("class", path.class);
    svg_path.setAttribute("stroke", "black");
    svg_path.setAttribute("stroke-width", "1");
    svg.appendChild(svg_path);
  });

  return svg;
}

import("contour").then((wasm) => {
  console.log("Contour wasm library loaded");

  const anchor = document.getElementById("svg");

  const inputElement = document.getElementById("img_input");
  inputElement.addEventListener("change", handleFiles, false);
  function handleFiles() {
    const fileList = this.files; /* now you can work with the file list */
    console.log(fileList);
    const reader = new FileReader();
    reader.onload = (ev: ProgressEvent<FileReader>): any => {
      if (typeof ev.target.result === "string") {
        return;
      }

      const bytes = new Uint8Array(ev.target.result);
      const isoline = wasm.isoline_from_tiff(
        bytes,
        Float32Array.from([25.0, 50.0, 75.0, 100.0])
      );

      anchor.innerHTML = "";
      anchor.appendChild(isoline_to_svg(isoline));
    };

    reader.readAsArrayBuffer(fileList[0]);
  }
});
