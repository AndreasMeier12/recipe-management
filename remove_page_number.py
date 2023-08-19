import re
import sqlite3

from dotenv import load_dotenv

load_dotenv()
con = sqlite3.connect('recipes.db')
cur = con.cursor()
res = cur.execute('SELECT * FROM recipe')
for row in res.fetchall():
    book_id = row[3]
    name = row[4]
    id = row[0]
    if book_id and book_id < 55:
        new_name = re.sub('\d+$', '', name).strip().replace('\'', '\'\'')
        statement = f'UPDATE recipe SET recipe_name=\'{new_name}\' WHERE recipe_id={id}'
        cur.execute(statement)
con.commit()
