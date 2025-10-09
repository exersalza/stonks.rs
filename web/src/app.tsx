import { useEffect, useRef, useState } from "preact/hooks";
import { Content } from "./comps/content";


const HIGHLIGHT_SIZE = (window.screen.height + window.screen.width) / 4

export function App() {
    const screen = window.screen;
    const maskId = "lines-circle-mask";

    const [[mX, mY], setMouse] = useState([0, 0]);
    const [mousePresent, setMousePresent] = useState(0);

    const screenRef = useRef<HTMLDivElement>(null);
    const stonksLineRef = useRef<SVGSVGElement>(null);

    const setMouseLoc = (e: MouseEvent) => {
        setMousePresent(1)
        setMouse([e.clientY - HIGHLIGHT_SIZE / 2, e.clientX - HIGHLIGHT_SIZE / 2])
    }

    const holyshitAMouseAppeared = () => {
        setMousePresent(1)
    }

    const mouseGone = () => {
        setMousePresent(0)
    }

    const renderSvg = () => {
        const svg = stonksLineRef.current;
        if (!svg) return;

        Array.from(svg.querySelectorAll("path")).forEach((el) => el.remove());

        const width = svg.clientWidth;
        const height = svg.clientHeight;
        const numberOfPoints = 50;
        const maxStep = height * 0.05;

        let points = [];
        let last_y = height / 2;

        for (let i = 0; i <= numberOfPoints; i++) {
            const x = (i * width) / numberOfPoints;
            const deltaY = (Math.random() * 2 - 1) * maxStep;
            last_y = Math.min(Math.max(last_y + deltaY, 0), height);
            points.push({ x, y: last_y });
        }

        let d = `M ${points[0].x} ${points[0].y}`;

        for (let i = 0; i < points.length - 1; i++) {
            const p0 = points[i === 0 ? i : i - 1];
            const p1 = points[i];
            const p2 = points[i + 1];
            const p3 = points[i + 2 < points.length ? i + 2 : i + 1];

            const cp1x = p1.x + (p2.x - p0.x) / 6;
            const cp1y = p1.y + (p2.y - p0.y) / 6;
            const cp2x = p2.x - (p3.x - p1.x) / 6;
            const cp2y = p2.y - (p3.y - p1.y) / 6;

            d += ` C ${cp1x} ${cp1y}, ${cp2x} ${cp2y}, ${p2.x} ${p2.y}`;
        }

        const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
        path.setAttribute("d", d);
        // path.setAttribute("stroke", "url(#lineGradient)");
        path.setAttribute("stroke", "white");
        path.setAttribute("stroke-width", "2");
        path.setAttribute("fill", "none");
        path.setAttribute("stroke-linecap", "round");
        path.setAttribute("stroke-linejoin", "round");

        path.setAttribute("mask", `url(#${maskId})`)

        svg.appendChild(path);
    }


    useEffect(() => {
        renderSvg()

        let refsda = screenRef.current;

        if (refsda) {
            refsda.addEventListener("mouseenter", holyshitAMouseAppeared)
            refsda.addEventListener("mouseleave", mouseGone)
        }

        window.addEventListener("resize", renderSvg)
        document.addEventListener("mousemove", setMouseLoc);

        return () => {
            document.removeEventListener("mousemove", setMouseLoc);
            window.removeEventListener("resize", renderSvg)

            if (refsda) {
                refsda.addEventListener("mouseenter", holyshitAMouseAppeared)
                refsda.addEventListener("mouseleave", mouseGone)
            }
        }
    }, [])

    return (
        <div className={"bg-black overflow-clip h-screen w-screen"} ref={screenRef}>
            <div className={"h-screen relative flex flex-col gap-0.25 overflow-hidden justify-center place-items-center z-10"}>
                {
                    Array.from({ length: screen.height / 80 }).map(() =>
                        <div className={"flex gap-0.25"}>
                            {
                                Array.from({ length: screen.width / 80 }).map(() =>
                                    <div className={"bg-black min-h-20 min-w-20 rounded-lg"}></div>
                                )
                            }
                        </div>
                    )
                }
            </div>

            <svg ref={stonksLineRef} className="h-screen w-screen absolute top-0 left-0 z-15 text-clip">
                <defs>
                    <radialGradient id="softMaskGradient" cx="50%" cy="50%" r="50%">
                        <stop offset="0" stop-color="white" stop-opacity="1" />
                        <stop offset="1" stop-color="white" stop-opacity=".01" />
                    </radialGradient>
                    <mask id={maskId}>
                        <rect x="0" y="0" width="100%" height="100%" fill="black" />
                        <circle
                            cx={mY + HIGHLIGHT_SIZE / 2}
                            cy={mX + HIGHLIGHT_SIZE / 2}
                            r={HIGHLIGHT_SIZE / 3.5}
                            fill="url(#softMaskGradient)"
                            style={{ opacity: mousePresent }}
                        />
                    </mask>
                </defs>
            </svg>

            <div className={"fixed bg-radial from-zinc-700 to-black to-70% rounded-full text-clip transition-opacity"} style={{
                top: mX,
                left: mY,
                height: HIGHLIGHT_SIZE,
                width: HIGHLIGHT_SIZE,
                opacity: mousePresent
            }} />

            <Content />
        </div>
    )
}
