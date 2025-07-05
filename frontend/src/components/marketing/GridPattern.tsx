import { useId } from "react";

interface Props {
  width?: number;
  height?: number;
  x?: string;
  y?: string;
  squares?: number[][];
  className?: string;
}

export default function GridPattern({ width = 20, height = 20, x = "-12", y = "4", squares, className }: Props) {
  const id = useId();
  return (
    <svg aria-hidden="true" className={className}>
      <defs>
        <pattern id={id} width={width} height={height} patternUnits="userSpaceOnUse" x={x} y={y}>
          <path d={`M.5 ${height}V.5H${width}`} fill="none" />
        </pattern>
      </defs>
      <rect width="100%" height="100%" strokeWidth={0} fill={`url(#${id})`} />
      {squares && (
        <svg x={x} y={y} className="overflow-visible">
          {squares.map(([sx, sy]) => (
            <rect key={`${sx}-${sy}`} width={width + 1} height={height + 1} x={sx * width} y={sy * height} />
          ))}
        </svg>
      )}
    </svg>
  );
} 