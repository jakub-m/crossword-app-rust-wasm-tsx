import React, { useState } from "react";
import './App.css';

interface TextInputProps {
    initialText?: string;
}

const TextInput: React.FC<TextInputProps> = ({initialText = ""}) => {
    const [text, setText] = useState(initialText);

    return (
        <div>
            <textarea
                value={text}
                onChange={(e) => setText(e.target.value)}
                placeholder="Enter text..."
                rows={10}
                cols={50}
                style={{ width: "100%", height: "200px" }} // TODO change this style
            />
            <p>Entered Text:</p>
            <pre>{text}</pre>
        </div>
    );
};

export default TextInput;

