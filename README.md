# 概要
競プロの副産物として書いたライブラリやツールをまとめました。


# AHC
ヒューリスティックコンテストで使用しているローカルテスタなどです。<br>run.py は https://note.com/aquatic_life/n/nbab11f703230 こちらを参考にしました。


# my_library
アルゴリズムコンテストなどで使用しているライブラリのうち、自分で実装したものをまとめました。
### dp
- ナップザックDP(knapsack.py)
- 最長増加部分列(LIS.py)
- 部分和DP(subset_sum.py)
### geometry
- 三角形の面積(is_triangle.py)
### miscellaneous
- マス目の余白を切り取る(cut4.py)
- 転倒数を数える(inversion_count.py)
- じゃんけんの勝敗判定(RSP.py)
### number
- MOD上の二項係数・順列の数え上げ、確立MODなど(mod_comb.py)
### search
- 幅優先探索(bfs.py)
- ダイクストラ法(dijkstra.py)
- トポロジカルソート(topological_sort.py)


# my_tools
自作したツールです。
### atcoder_template_generator.js
AtCoderの問題文のページ上に、テンプレートファイルをダウンロードするボタンを追加するスクリプトです。(Tampermonkeyで動作)<br>ほぼ全てChatGPTに実装してもらいました。  
<img src="my_tools/2023-10-12 18-28-51.gif" width="800">


# template_py
Pythonのテンプレートです。


# template_rs
Rustのテンプレートです。
