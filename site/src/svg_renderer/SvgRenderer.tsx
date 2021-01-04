import * as React from "react";
import { SvgRendererProps } from "./SvgRenderer.types";

export const SvgRenderer = (props: SvgRendererProps) => {
  const { contour } = props;

  const { paths, view_box } = contour;

  return (
    <svg viewBox={view_box} width="100%" height="100%">
      {paths.map((path, i) => (
        <path
          d={path.path}
          fill={path.fill}
          className={path.class}
          stroke="black"
          strokeWidth={1}
          key={i}
        />
      ))}
    </svg>
  );
};
