import React, {useState, useEffect} from 'react';
import './App.css';
import { Word } from './Word';
import { CrosswordGrid } from './Grid';
import { generate_crossword_js } from './crossword_wasm/crossword'
import { DefinitionArea } from './DefinitionArea';
import { get_text, TextId, Lang, cycle_lang } from './Text';
import InputGroup from 'react-bootstrap/InputGroup';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import Stack from 'react-bootstrap/Stack';

type GeneratorMode = "InputOrder" | "Automatic";

function App() {
  const [crosswordWords, setCrosswordWords] = useState<Word[]>([])
  const [isHiddenForPrint, setIsHiddenForPrint] = useState<boolean>(false);
  const [generatorMode, setGeneratorMode] = useState<GeneratorMode>("InputOrder")
  const [lang, setLang] = useState<Lang>("EN")
  const [textInForm, setTextInForm] = useState<string>(get_text(TextId.InitialText, lang))
  const [textUsedForCrossword, setTextUsedForCrossword] = useState<string>(get_text(TextId.InitialText, lang))
  const [droppedWords, setDroppedWords] = useState<string[]>([])

  useEffect(() => {
    let {words: cwords, dropped} = generate_crossword_from_input(textUsedForCrossword, generatorMode)
    setCrosswordWords(cwords)
    setDroppedWords(dropped)
  }, [textUsedForCrossword, generatorMode])

  const onKeyDownInForm = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && e.shiftKey === false) {
      setTextUsedForCrossword(textInForm)
    }
  }

  useEffect(() => {
    if (!isHiddenForPrint) {
      return
    }
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsHiddenForPrint(false)
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [isHiddenForPrint, setIsHiddenForPrint]);

  const get_next_mode = (prev: GeneratorMode): GeneratorMode => {
    if (prev === "Automatic") {
      return "InputOrder"
    }
    return "Automatic"
  }

  const mode_to_text: Record<GeneratorMode, string> = {
    "InputOrder": get_text(TextId.ModeInputOrder, lang),
    "Automatic": get_text(TextId.ModeAutomatic, lang),
  }

  const input_form = (
    <>
      <InputGroup>
        <Form.Control
            as="textarea"
            placeholder={get_text(TextId.EnterWord, lang)}
            onChange={(e) => setTextInForm(e.target.value)}
            onKeyDown={onKeyDownInForm}
            value={textInForm}
            style={
              {
                height: "33vh"
              }
            }
        />
      </InputGroup>
      <ButtonGroup>
        <Button id="lang_but" variant="outline-primary" onClick={() => setLang((prev) => cycle_lang(prev))}>
          {
            {"PL": "🇵🇱", "EN": "🇬🇧"}[lang]
          }
        </Button>
        <Button id="mode_selector" variant="outline-primary" onClick={() => setGeneratorMode((prev_mode) => get_next_mode(prev_mode))}>{mode_to_text[generatorMode]}</Button>
        <Button id="hide_for_print" variant="outline-primary" value="1" onClick={(_) => setIsHiddenForPrint((prev) => !prev)}>
            {get_text(TextId.HideForPrint, lang)}
        </Button>
      </ButtonGroup>
    </>
  )

  const footer = (
    <footer>
      <a style={{fontSize: "0.7em", color: "#333", textDecoration: "none"}} href="https://github.com/jakub-m/crossword-app-rust-wasm-tsx">[jakub-m]</a>
    </footer>
  )

  const dropped_words_area = (() => {
    if (droppedWords.length === 0) {
      return null
    }
    return (<Stack>
        <div>{get_text(TextId.DroppedWords, lang)}</div>
        {droppedWords.map((w) => <div>{w}</div>)}
    </Stack>)
  })();


  if (isHiddenForPrint) {
    return (
      <Container onClick={() => setIsHiddenForPrint(false)}>
          <div style={{padding: "2em"}}>
            <CrosswordGrid words={crosswordWords} hideLetters={true}></CrosswordGrid>
            <DefinitionArea words={crosswordWords} lang={lang}/>
          </div>
      </Container>
    )
  } else {
    return (
      <Container>
          {input_form}
          <div style={{padding: "2em"}}>
            <CrosswordGrid words={crosswordWords} hideLetters={false}></CrosswordGrid>
            <Stack gap={3}>
              <DefinitionArea words={crosswordWords} lang={lang}/>
              {dropped_words_area}
            </Stack>
          </div>
          {footer}
      </Container>
    )
  }

}

const generate_crossword_from_input = (text: string, mode: string) : {words: Word[], dropped: string[]} => {
  const input_definitions: Record<string, string> = text
    .split("\n")
    .map(line => line.trim())
    .filter(line => line.length > 0)
    .reduce((acc, line) => {
        const [first, ...rest] = line.split(/\s+/);
        const firstLc = first.toLowerCase();
        acc[firstLc] = rest.join(" ");
        return acc
    }, {} as Record<string, string>);
  
  const output_cwords = generate_crossword_js(Object.keys(input_definitions), mode)
  const words = output_cwords.map((w) => {
    return {
      id: w.id,
      x: w.x,
      y: w.y,
      word: w.word,
      orientation: w.orientation,
      definition: input_definitions[w.word],
    } as Word
  });

  const output_word_set = new Set(words.map((w) => w.word));
  const dropped = Object.keys(input_definitions).filter((s) => !output_word_set.has(s));
  return {words, dropped}
}

export default App;
