import React from "react";

// WhisperFly wordmark (replaces the upstream Handy logo; upstream credit
// lives in About + NOTICE.md). Same props contract as the original.
const HandyTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 930 328"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* lightning bolt */}
      <path
        d="M120 30 L52 190 H104 L84 298 L188 132 H128 L156 30 Z"
        fill="currentColor"
      />
      <text
        x="225"
        y="215"
        fontFamily="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
        fontSize="150"
        fontWeight="700"
        fill="currentColor"
      >
        {"WhisperFly"}
      </text>
    </svg>
  );
};

export default HandyTextLogo;
