import { useEffect, useRef, useState } from "preact/hooks";


const HIGHLIGHT_SIZE = (window.screen.height + window.screen.width) / 6

export function App() {
    const screen = window.screen;

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

        Array.from(svg.querySelectorAll("line")).forEach((el) => el.remove());

        const width = svg.clientWidth;
        const height = svg.clientHeight;
        const numberOfLines = 50;

        for (let i = 0; i < numberOfLines; i++) {
            const x1 = Math.random() * width;
            const y1 = Math.random() * height;
            const x2 = Math.random() * width;
            const y2 = Math.random() * height;

            const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
            line.setAttribute("x1", x1);
            line.setAttribute("y1", y1);
            line.setAttribute("x2", x2);
            line.setAttribute("y2", y2);
            line.setAttribute("stroke", "green");
            line.setAttribute("mask", "url(#test)");
            line.setAttribute("stroke-width", "2");

            svg.appendChild(line);
        }
    }


    useEffect(() => {
        renderSvg()
        let refsda = screenRef.current;

        if (refsda) {
            refsda.addEventListener("mouseenter", holyshitAMouseAppeared)
            refsda.addEventListener("mouseleave", mouseGone)
        }

        document.addEventListener("mousemove", setMouseLoc);

        return () => {
            document.removeEventListener("mousemove", setMouseLoc);

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


            <svg
                ref={stonksLineRef}
                className={"h-screen w-screen absolute top-0 left-0 z-15 text-clip"}
            >
                <mask x={mY} y={mX} id="test">
                    <rect width="100%" height="100%" fill="white" />
                    <rect x="100" y="50" width="100" height="100" fill="black" />
                </mask>
            </svg>


            <div className={"fixed bg-radial from-zinc-700 to-black to-70% rounded-full text-clip transition-opacity"} style={{
                top: mX,
                left: mY,
                height: HIGHLIGHT_SIZE,
                width: HIGHLIGHT_SIZE,
                opacity: mousePresent
            }} />

        </div>
    )
}
