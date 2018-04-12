# tsvhtml

Converts [TSV](https://en.wikipedia.org/wiki/Tab-separated_values to HTML.

Reads a table of data formatted as [table separated values (TSV)](https://en.wikipedia.org/wiki/Tab-separated_values) and writes out as an HTML table that can more easilly be displayed in a web browser.

## Examples

```
$ tsvhtml.exe << EOF
a	b	c
1	2	3
EOF
<head></head><body><table border="1"><tr><th>a</th><th>b</th><th>c</th></tr><tr><td>1</td><td>2</td><td>3</td></tr></td></tr></table></body>
```






