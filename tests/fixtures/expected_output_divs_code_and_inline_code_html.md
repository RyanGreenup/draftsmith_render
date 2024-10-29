<h1>Heading</h1>
<h2>Details Boxes</h2>
<p>This is some text</p>
<p>this latent will be preserved:
:::</p>
<div role="alert" class="alert alert-info">
<p>Some content</p>
</div>
<p>A trailing block:</p>
<p>:::</p>
<p>Some other text</p>
<div class="alert alert-info">
<p>More content</p>
<div class="tip">
<p>This is nested</p>
</div>
</div>
<p>A latent block
:::</p>
<h2>Code Blocks</h2>
<h3>Hidden</h3>
<h3>Display</h3>
<div class="rhai-display">
<pre lang="rust"><code>let t = &quot;&quot;;
for i in 1..s {
    t += i;
    if i != (s-1) {
        t += &quot; + &quot;;
    }
}
t
</code></pre>
<div class="rhai-out">
<pre><code>1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20 + 21 + 22 + 23 + 24 + 25 + 26 + 27 + 28 + 29 + 30 + 31 + 32 + 33 + 34 + 35 + 36 + 37 + 38 + 39 + 40 + 41 + 42 + 43 + 44
</code></pre>
</div>
</div>
<h3>Inline Code</h3>
<p>The sum of the first 10 numbers is 45 all together that is: 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20 + 21 + 22 + 23 + 24 + 25 + 26 + 27 + 28 + 29 + 30 + 31 + 32 + 33 + 34 + 35 + 36 + 37 + 38 + 39 + 40 + 41 + 42 + 43 + 44</p>

