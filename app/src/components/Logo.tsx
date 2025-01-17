export function IceblinkLogo({
  size = 60,
  fancy = false,
}: {
  size?: number;
  fancy?: boolean;
}) {
  return (
    <svg
      width="1440"
      height="1440"
      viewBox="0 0 1440 1440"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      style={{
        width: size,
        height: size,
        borderRadius: 12,
        borderTop: fancy ? "1px solid rgba(255, 255, 255, 0.20)" : undefined,
        boxShadow: fancy
          ? "0px 0px 0px 1px rgba(0, 0, 0, 0.25), 0px 5px 50px 0px rgba(0, 0, 0, 0.25)"
          : undefined,
        margin: fancy ? 1 : 0,
      }}
    >
      <rect width="1440" height="1440" fill="#251E41" />
      <path
        d="M635.4 293.417C681.934 266.55 739.266 266.55 785.8 293.417L1042.43 441.583C1088.97 468.449 1117.63 518.101 1117.63 571.833V868.167C1117.63 921.899 1088.97 971.55 1042.43 998.417L785.8 1146.58C739.266 1173.45 681.934 1173.45 635.4 1146.58L378.768 998.417C332.234 971.55 303.568 921.899 303.568 868.167V571.833C303.568 518.101 332.234 468.449 378.768 441.583L635.4 293.417Z"
        fill="url(#paint0_linear_898_135)"
      />
      <path
        d="M743.713 697.197L615.166 563.528L567.738 609.138L696.287 742.807L743.713 697.197ZM615.166 876.475L743.713 742.807L696.287 697.197L567.738 830.866L615.166 876.475ZM461.5 625.428V814.576H527.3V625.428H461.5ZM872.262 830.866L743.713 697.197L696.287 742.807L824.834 876.475L872.262 830.866ZM912.7 625.428V814.576H978.5V625.428H912.7ZM743.713 742.807L872.262 609.138L824.834 563.528L696.287 697.197L743.713 742.807ZM978.5 625.428C978.5 545.027 880.566 505.576 824.834 563.528L872.262 609.138C886.928 593.888 912.7 604.269 912.7 625.428H978.5ZM824.834 876.475C880.566 934.427 978.5 894.977 978.5 814.576H912.7C912.7 835.734 886.928 846.116 872.262 830.866L824.834 876.475ZM567.738 830.866C553.072 846.116 527.3 835.734 527.3 814.576H461.5C461.5 894.977 559.434 934.427 615.166 876.475L567.738 830.866ZM615.166 563.528C559.434 505.576 461.5 545.027 461.5 625.428H527.3C527.3 604.269 553.072 593.888 567.738 609.138L615.166 563.528Z"
        fill="#E2D4FF"
      />
      <defs>
        <linearGradient
          id="paint0_linear_898_135"
          x1="240.6"
          y1="250"
          x2="229.123"
          y2="1178.24"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor="#8469E4" />
          <stop offset="1" stopColor="#5135CE" />
        </linearGradient>
      </defs>
    </svg>
  );
}
