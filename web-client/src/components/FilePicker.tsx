import Dropzone from "shadcn-dropzone";
import { MdOutlineCloudUpload } from "react-icons/md";
import { FaRegFileAlt } from "react-icons/fa";
import { IoTrash } from "react-icons/io5";
import { Avatar, AvatarFallback } from "./ui/avatar";
import { Button } from "./ui/button";
import { Item, ItemContent, ItemTitle, ItemDescription, ItemActions } from "./ui/item";

interface AuthProviderProps {
    handleCallBack: (files: File[]) => void;
    accept: any
    file: File | null | undefined;
    setFile: React.Dispatch<React.SetStateAction<File | null | undefined>>;
}

function formatBytes(bytes: number) {
    if (bytes === 0) return "0 B";

    const units = ["B", "KB", "MB", "GB", "TB"];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return (bytes / Math.pow(k, i)).toFixed(2) + " " + units[i];
}

export default function FilePicker({ handleCallBack, accept, file, setFile }: AuthProviderProps) {
    return (
        <>
            {!file ?
                <Dropzone
                    accept={accept}
                    dropZoneClassName="flex flex-col items-center border-2 border-dashed p-6 rounded-lg w-full"
                    onDrop={(files: File[]) => handleCallBack(files)}
                >
                    {() => (
                        <>
                            <MdOutlineCloudUpload className="text-5xl" />
                            <div className="text-sm font-medium">
                                Click or drag file to upload
                            </div>
                        </>
                    )}
                </Dropzone>
                :
                <Item variant="outline" className="w-full">
                    <Avatar>
                        <AvatarFallback><FaRegFileAlt /></AvatarFallback>
                    </Avatar>
                    <ItemContent>
                        <ItemTitle>{file.name}</ItemTitle>
                        <ItemDescription>
                            {formatBytes(file.size)}
                        </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                        <Button variant="outline" size="sm" className="text-red-400" onClick={() => { setFile(null) }}>
                            <IoTrash />
                        </Button>
                    </ItemActions>
                </Item>}
        </>
    );
}