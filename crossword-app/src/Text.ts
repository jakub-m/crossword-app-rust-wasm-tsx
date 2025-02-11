enum TextId {
  DroppedWords,
  EnterWord,
  Generate,
  GenerateOnEnter,
  HideForPrint,
  Horizontal,
  InitialText,
  ModeAutomatic,
  ModeInputOrder,
  Vertical,
  Words,
}

const initialText = `boar boar
robot not human
bird flies
car to drive
trap is's a
`;

const EN: Record<TextId, string> = {
  [TextId.DroppedWords]: "Dropped words",
  [TextId.EnterWord]: "Enter word, followed by it's definition",
  [TextId.GenerateOnEnter]: "Generate on Enter",
  [TextId.Generate]: "Generate",
  [TextId.HideForPrint]: "Hide for print (ESC or click to go back)",
  [TextId.Horizontal]: "Horizontal",
  [TextId.InitialText]: initialText,
  [TextId.ModeAutomatic]: "Automatic",
  [TextId.ModeInputOrder]: "Use input order",
  [TextId.Vertical]: "Vertical",
  [TextId.Words]: "Words",
};

const PL: Record<TextId, string> = {
  [TextId.DroppedWords]: "Nie pasujące słowa",
  [TextId.EnterWord]: "Wpisz tekst i definicję",
  [TextId.GenerateOnEnter]: "Generuj na Enter",
  [TextId.Generate]: "Generuj",
  [TextId.HideForPrint]: "Widok do druku (ESC albo kliknij żeby wrócić)",
  [TextId.Horizontal]: "Poziomo",
  [TextId.InitialText]: initialText,
  [TextId.ModeAutomatic]: "Tryb Automatyczny",
  [TextId.ModeInputOrder]: "W kolejności",
  [TextId.Vertical]: "Pionowo",
  [TextId.Words]: "Słowa",
};

type Lang = "EN" | "PL";

const cycle_lang = (lang: Lang): Lang => {
  if (lang === "EN") {
    return "PL";
  }
  return "EN";
};

const get_text = (text_id: TextId, lang: Lang): string => {
  if (lang === "PL") {
    return PL[text_id];
  }
  return EN[text_id];
};

export { TextId, get_text, Lang, cycle_lang };
