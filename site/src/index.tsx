import * as React from "react";
import * as ReactDOM from "react-dom";

import { IsoLineGenerator } from "./isoline_generator";

import("contour").then((wasm) => {
  console.log("Contour wasm library loaded");

  console.log("Initializing react");
  const domContainer = document.querySelector("#app");
  ReactDOM.render(
    <IsoLineGenerator contourFunction={wasm.isoline_from_tiff} />,
    domContainer
  );
});
