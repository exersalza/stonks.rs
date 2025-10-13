import {  useState } from "preact/hooks"
import githubMarkWhite from "/github-mark-white.svg"
import rat from "/rat-dance.gif"

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


const Rats = ({ engaged }: { engaged: boolean }) => {
    if (engaged) {
        return (
            <div className={"h-screen w-screen pointer-events-none absolute flex justify-between"}>
                <div>
                    <img src={rat} />
                    <img src={rat} />
                </div>
                <div>
                    <img src={rat} />
                    <img src={rat} />
                </div>
            </div>
        )
    }


    return (
        <div></div>
    )
}


export const Content = () => {
    const [ratsEngaged, setRatsEngaged] = useState<boolean>(false);


    return (
        <div className={"absolute h-screen w-screen z-50 top-0 left-0 grid place-content-center"}>
            <div className={"border-1 border-white rounded-lg bg-black/90 transition-all"}>
                <div className={"flex text-white flex-col p-4 gap-4"}>
                    <h1 className={"text-xl text-center"}><code>$ ./stonks.rs</code></h1>
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

                    <details className={"transition-all"}>
                        <summary>Preview</summary>
                        <p>no example yet ... windows 11 is sabotaging me</p>
                    </details>

                    <ol className={"list-disc mx-4"}>
                        <li>Can display as many Chains as your screen supports.</li>
                        <li>Two prebuilt layouts (Master, Splace)</li>
                        <li>Coinbase and Kraken apis</li>
                        <li>Vim motion bindings</li>
                    </ol>
                    <div className={"flex gap-2 justify-center"}>
                        <input
                            id="engageRats"
                            type="checkbox"
                            checked={ratsEngaged}
                            onClick={() => setRatsEngaged(prev => !prev)} />
                        <label for="engageRats" className={"select-none"}>engage rats</label>
                    </div>
                </div>
            </div>

            <Rats engaged={ratsEngaged} />
        </div>
    )
}
