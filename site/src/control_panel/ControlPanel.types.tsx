export interface Control {
  threshold: number;
  strokeWidth: number;
  strokeColour: string;
}

export interface ControlPanelProps {
  controls: Control[];
  onDeleteControl: (index: number) => void;
}
