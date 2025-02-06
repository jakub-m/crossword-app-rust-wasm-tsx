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


type GeneratorMode = "InputOrder" | "Automatic";


function App() {
  const [crosswordWords, setCrosswordWords] = useState<Word[]>([])
  const [isHiddenForPrint, setIsHiddenForPrint] = useState<boolean>(false);
  const [generatorMode, setGeneratorMode] = useState<GeneratorMode>("InputOrder")
  const [lang, setLang] = useState<Lang>("EN")
  const [textInForm, setTextInForm] = useState<string>(get_text(TextId.InitialText, lang))
  const [textUsedForCrossword, setTextUsedForCrossword] = useState<string>(get_text(TextId.InitialText, lang))

  useEffect(() => {
    let cwords = generate_crossword_from_input(textUsedForCrossword, generatorMode)
    setCrosswordWords(cwords)
  }, [textUsedForCrossword, generatorMode])

  // todo set text on Enter

  const onKeyDownInForm = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && e.shiftKey === false) {
      setTextUsedForCrossword(textInForm)
    }
  }

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsHiddenForPrint(false)
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [setIsHiddenForPrint]);


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

  return (
  <Container>
      {isHiddenForPrint ? null : input_form}
      <div style={{padding: "2em"}}>
        <CrosswordGrid words={crosswordWords} hideLetters={isHiddenForPrint}></CrosswordGrid>
        <DefinitionArea words={crosswordWords} lang={lang}/>
      </div>
  </Container>)
}


const generate_crossword_from_input = (text: string, mode: string) : Word[] => {
  const input_words: Record<string, string> = text
    .split("\n")
    .map(line => line.trim())
    .filter(line => line.length > 0)
    .reduce((acc, line) => {
        const [first, ...rest] = line.split(/\s+/);
        acc[first] = rest.join(" ");
        return acc
    }, {} as Record<string, string>);
  
  const output_cwords = generate_crossword_js(Object.keys(input_words), mode)
  return output_cwords.map((w) => {
    return {
      id: w.id,
      x: w.x,
      y: w.y,
      word: w.word,
      orientation: w.orientation,
      definition: input_words[w.word],
    } as Word
  });
}

export default App;
