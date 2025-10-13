import { useEffect, useState } from "preact/hooks"
import githubMarkWhite  from "/github-mark-white.svg"

const PROJECT_LINK = "http://github.com/exersalza/stonks.rs";

export const Badge = ({ title, content }: { title?: any, content: any }) => {
    return (
        <div className={"border-white border-1 rounded-lg select-none flex gap-1"}>
            {
                title ?
                    <p className={"bg-zinc-600 rounded-[6px] px-2 flex justify-center items-center"}>
                        {title}
                    </p> : ""
            }
            <p className={"pr-2"}>
                {content}
            </p>
        </div>
    )
}


export const Content = () => {
    return (
        <div className={"absolute h-screen w-screen z-50 top-0 left-0 grid place-content-center"}>
            <div className={"border-1 border-white rounded-lg bg-black/90"}>
                <div className={"flex text-white flex-col p-4 gap-4"}>
                    <h1 className={"text-xl"}><code>$ ./stonks.rs</code></h1>
                    <div className={"flex gap-2"}>
                        {
                            [
                                ["tui", <a href="https://ratatui.rs" className={"underline"}>ratatui</a>],
                                ["lang", "rust 🦀🚀"],
                                [<img src={githubMarkWhite} className={'size-5'} />, <a href={PROJECT_LINK} className={"underline"}>github</a>]
                            ].map((ele, i) => {
                                let e: any = ele;
                                let t: any = "";

                                if (ele instanceof Array) {
                                    e = ele[1];
                                    t = ele[0];
                                }

                                return (
                                    <Badge title={t} content={e} key={i} />
                                )
                            })
                        }
                    </div>
                    <ol>
                        <li>some</li>
                        <li>more</li>
                        <li>omg</li>
                        <li>stop</li>
                        <li>please</li>
                    </ol>
                </div>
            </div>
        </div>
    )
}
