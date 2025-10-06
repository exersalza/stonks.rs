import { useState } from 'preact/hooks'
import preactLogo from './assets/preact.svg'
import viteLogo from '/vite.svg'
import './app.css'

export function App() {
    const screen = window.screen;

    return (
        <div className={"h-screen flex flex-col gap-0.25 overflow-hidden"}>
            {
                Array.from({ length: screen.height / 80 }).map(() =>
                    <div className={"flex gap-0.25"}>
                        {
                            Array.from({ length: screen.width / 80 }).map(() =>
                                <div className={"bg-black min-h-20 min-w-20"}></div>
                            )
                        }
                    </div>
                )
            }
        </div >
    )
}
