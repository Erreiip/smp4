import FilePicker from "@/components/FilePicker";
import { Page } from "./Page";

export default function Uploader() {
    return (
        <Page title="Upload" desc="Upload a file you wish to sign">
            <div className="min-h-xl flex items-center flex-col justify-center">
                <FilePicker />
            </div >
        </Page>
    )
}
