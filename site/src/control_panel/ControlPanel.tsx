import * as React from "react";
import { Control, ControlPanelProps } from "./ControlPanel.types";

const Control = (props: Control) => {
  const { threshold, strokeWidth, strokeColour } = props;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
      }}
    >
      <span>{threshold}</span>
      <span>{strokeWidth}</span>
      <span>{strokeColour}</span>
    </div>
  );
};

export const ControlPanel = (props: ControlPanelProps) => {
  const { controls, onDeleteControl } = props;

  return (
    <>
      {controls.map((control, i) => (
        <span key={`${control.threshold}_${i}`}>
          <Control {...control} />
          <button onClick={() => onDeleteControl(i)}>Delete</button>
        </span>
      ))}
    </>
  );
};
