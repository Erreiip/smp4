import FileUploader from "@/components/FileUploader";
import { PageLayout } from "./PageLayout";

export default function Uploader() {
    return (
        <PageLayout title="File Signer" desc="
            Upload a file you wish to sign, then fill up the required field.
            <br />
            Once the file is generated, this information can no longer be modified. Sfile guarantees the integrity of the information within the file.
        ">
            <div className="min-h-xl flex items-center flex-col justify-center">
                <FileUploader />
            </div >
        </PageLayout>
    )
}
