import { Button } from "@/components/ui/button"
import { useState } from "react";
import { Card, CardDescription, CardFooter, CardHeader, CardTitle } from "./ui/card";
import Dropzone from "shadcn-dropzone";
import { IoCloudUploadSharp, IoTrash } from "react-icons/io5";
import { Item, ItemContent, ItemTitle, ItemDescription, ItemActions } from "./ui/item";
import { Avatar, AvatarFallback } from "./ui/avatar";
import { FaRegFileAlt } from "react-icons/fa";
import { Field, FieldGroup, FieldLabel } from "./ui/field";
import { Input } from "./ui/input";
import { Textarea } from "./ui/textarea";

function formatBytes(bytes: number) {
    if (bytes === 0) return "0 B";

    const units = ["B", "KB", "MB", "GB", "TB"];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return (bytes / Math.pow(k, i)).toFixed(2) + " " + units[i];
}

export default function FilePicker() {
    const [file, setFile] = useState<File | null>();
    const [formData, setFormData] = useState({
        title: '',
        desc: ''
    })

    const handleFileDrop = (files: File[]) => {
        if (files.length != 1)
            throw new Error('Single file required')
        let file = files[0]
        setFile(file)
        console.log(file)
    };
    const handleChange = (e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {
        const { id, value } = e.target
        setFormData((prev) => ({ ...prev, [id]: value }))
    }

    return (
        <div className="min-h-xl flex items-center flex-col justify-center">
            <Card className="w-4xl">
                <CardHeader>
                    <CardTitle>Upload you file here</CardTitle>
                </CardHeader>
                <CardDescription className="flex gap-6 flex-col px-6 [&>div]:w-full">
                    <Dropzone
                        dropZoneClassName="flex flex-col items-center border-2 border-dashed p-6 rounded-lg w-full"
                        onDrop={(files: File[]) => handleFileDrop(files)}>
                        {() => (
                            <>
                                <IoCloudUploadSharp className='text-5xl' />
                                <div className='text-sm font-medium'>
                                    Click or drag file to upload
                                </div>
                            </>
                        )}
                    </Dropzone>
                    {file ?
                        <div className="flex w-full flex-col">
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
                            </Item>
                            <FieldGroup className="mt-6">
                                <Field>
                                    <FieldLabel htmlFor="title">Title</FieldLabel>
                                    <Input id="title" type="text" placeholder="Title" value={formData.title} onChange={handleChange} />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="desc">Description</FieldLabel>
                                    <Textarea id="desc" placeholder="Description" value={formData.desc} onChange={handleChange} />
                                </Field>
                            </FieldGroup>
                        </div>
                        : <></>
                    }

                </CardDescription>
                <CardFooter className="flex gap-6">
                    <Button className="flex-1" variant="outline" id="clear-btn" onClick={() => { setFile(null) }}>Clear</Button>
                    <Button className="flex-1">Upload</Button>
                </CardFooter>
            </Card>
        </div >
    )
}
