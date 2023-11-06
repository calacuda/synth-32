from requests import get
import pandas as pd


url = "https://pages.mtu.edu/~suits/notefreqs.html"
page = get(url).content
df = pd.read_html(page)[1]


# print(df.info)
notes = [row[0].split("/")[0] for _, row in df.iterrows()]
s_quote = "'"
d_quote = '"'
print(f"pub const NOTE_NAMES: [&str; {len(notes)}] = {str(notes).replace(s_quote, d_quote)};")


