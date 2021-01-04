import * as React from "react";
import { IsolineGeneratorProps, IsolineSVG } from "./IsolineGenerator.types";
import { SvgRenderer } from "../svg_renderer";
import { Control, ControlPanel } from "../control_panel";

export const IsoLineGenerator = (props: IsolineGeneratorProps) => {
  const { contourFunction } = props;

  const [currContour, setContour] = React.useState<IsolineSVG>();
  const [thresholds, setThresholds] = React.useState<Control[]>([
    { threshold: 25.0, strokeWidth: 1, strokeColour: "black" },
    { threshold: 50.0, strokeWidth: 1, strokeColour: "black" },
    { threshold: 75.0, strokeWidth: 1, strokeColour: "black" },
    { threshold: 100.0, strokeWidth: 1, strokeColour: "black" },
  ]);

  const fileOnLoad = React.useCallback(
    (ev: ProgressEvent<FileReader>) => {
      if (typeof ev.target.result === "string") {
        return;
      }
      const bytes = new Uint8Array(ev.target.result);
      setContour(
        contourFunction(
          bytes,
          Float32Array.from(thresholds.map((control) => control.threshold))
        )
      );
    },
    [thresholds, contourFunction]
  );
  const fileReader = React.useMemo(() => {
    const fileReader = new FileReader();
    fileReader.onload = fileOnLoad;

    return fileReader;
  }, [fileOnLoad]);

  const onChange = React.useCallback(
    (ev: React.ChangeEvent<HTMLInputElement>) => {
      const currFile = ev.target.files[0];

      fileReader.readAsArrayBuffer(currFile);
    },
    [fileReader, contourFunction]
  );

  const onDeleteControl = React.useCallback(
    (i: number) => {
      setThresholds((t) => t.filter((_, index) => index !== i));
    },
    [setThresholds]
  );

  return (
    <div
      style={{
        display: "flex",
        width: "50%",
        justifySelf: "center",
        flexDirection: "column",
      }}
    >
      <input type="file" id="img_input" onChange={onChange} />
      <ControlPanel controls={thresholds} onDeleteControl={onDeleteControl} />

      {currContour && <SvgRenderer contour={currContour} />}
    </div>
  );
};
