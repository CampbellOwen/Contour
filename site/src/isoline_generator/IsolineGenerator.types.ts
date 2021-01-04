export interface IsolinePath {
  class: string;
  path: string;
  fill: string;
}

export interface IsolineSVG {
  paths: [IsolinePath];
  view_box: string;
}

export interface IsolineGeneratorProps {
  contourFunction: (data: Uint8Array, thresholds: Float32Array) => IsolineSVG;
}
