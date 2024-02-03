import { Message } from "primereact/message"
import { open } from '@tauri-apps/api/shell';

export const OverviewWarning = () => {
    const onHrefClick = () => open("https://github.com/Atsukoro1/vrchat-rpc");

    return (
        <button className="w-full" type="button" onClick={onHrefClick}>
            <Message
                pt={{
                    root: {
                        style: {
                            height: "32px",
                        }
                    },
                    text: {
                        style: {
                            fontSize: 14,
                        },
                    },
                }}
                className="w-full"
                severity="warn"
                text="VRChat RPC is still in BETA, report all bugs on the github"
            />
        </button>
    )
}