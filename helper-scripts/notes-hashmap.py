from requests import get
import pandas as pd


url = "https://pages.mtu.edu/~suits/notefreqs.html"
page = get(url).content
df = pd.read_html(page)[1]

# print(df.info)
for _, row in df.iterrows():
    # print(row)
    (name, frequency) = row[0], row[1]
    for note in name.split("/"):
        print(f"notes.insert(\"{note}\", {frequency});")
