import React from "react"
import {Word} from "./Word"
import Stack from 'react-bootstrap/Stack';
import { get_text, TextId, Lang } from "./Text";


interface DefinitionAreaProps {
    words: Word[]
    lang: Lang
}

const DefinitionArea: React.FC<DefinitionAreaProps> = ({words, lang}) => {
    const word_to_div = (w: Word) =><div key={w.id}>{w.id}: {w.definition}</div>;
    words.sort((a, b) => a.id - b.id)
    return (
        <Stack direction="horizontal">
            <Stack>
                <div>{get_text(TextId.Horizontal, lang)}</div>
                {words.filter((w) => w.orientation === "hor").map((w) => word_to_div(w))}
            </Stack>
            <Stack>
                <div>{get_text(TextId.Vertical, lang)}</div>
                {words.filter((w) => w.orientation === "ver").map((w) => word_to_div(w))}
            </Stack>
        </Stack>
    )

}

export {DefinitionArea}