# Test combinations of features

Command  ← {"cargo" "c" "--lib" "--no-default-features"}
Features ← (
  {"audio" "tls"}
  ⊂⨬(
    {"gif,image,terminal_image,webcam" "lsp,raw_mode"}
  | {"audio_encode" "gif" "image" "terminal_image" "lsp" "native_sys" "raw_mode"}
  ) ∈:□"all" &args
)

&p "Checking with no features"
⍤⟜≍: 0 &runi Command

↘1⋯⇡ⁿ:2⧻. Features
≡(/$"_,_" ▽
  &p $"Checking with features: _".
  ⊂ Command ⊂ □"--features" □
  ⍤⟜≍: 0 &runi
)⊙¤
