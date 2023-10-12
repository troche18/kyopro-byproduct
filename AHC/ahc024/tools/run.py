##run.py##
import os
import subprocess
import shutil
import time
import multiprocessing
from math import log1p

# 並列処理するときのプロセス数
num_processes = 10

# ビルドするC++ファイル名
py_file_name = "ahc024.py"
# 出力結果を記入するファイル名
output_file = "result/result.txt"


def init():
    # 出力結果を格納するフォルダ作成
    if not os.path.exists("out"):
        os.makedirs("out")
    if not os.path.exists("result"):
        os.makedirs("result")
    if not os.path.exists("result/visualizer"):
        os.makedirs("result/visualizer")

    # 既存結果の消去
    with open(output_file, "w") as f:
        f.write("")
    with open("result/result_score.txt", "w") as f:
        f.write("")
    return


# inフォルダ内にある.txtファイルのリストを返す
def get_txt_list():
    # inフォルダにある入力用の.txtファイルをすべて読み込む
    files = os.listdir("in")
    txt_files = [f for f in files if f.endswith(".txt")]
    return txt_files


# 入力がfile_nameであるファイルに対するC++実行ファイルを実行 返り値: 実行時間
def exec_cpp(file_name):
    run_time = 0.0
    # inフォルダにあるtxtファイルの入力に対し、コードを実行する。出力結果はoutフォルダにinフォルダのファイル名と同じ名称で保存
    with open("in/" + file_name, "r") as f_in:
        with open("out/" + file_name, "w") as f_out:
            start_time = time.perf_counter()
            subprocess.run(["pypy", py_file_name], stdin=f_in, stdout=f_out)
            end_time = time.perf_counter()
            run_time = end_time - start_time
    return run_time


# 入力がfile_nameであるテストケースの結果を出力
def output(file_name, run_time):
    with open(output_file, "a") as f_out:
        # TestCaseの名称を出力
        f_out.write("\nTest Case: " + str(file_name) + "\n")
        f_out.write("Run Time is: " + str(run_time) + "[s]\n")
        # TestCaseのテキストファイル1行目を出力(入力データのサイズであることが多いため)(コンテストの入力に応じて変えること)
        # with open("in/" + file_name, "r") as f_in:
        #     line = f_in.readline()
        #     f_out.write(line)

    # 公式配布のRustを実行&結果の書き込み
    res = subprocess.run(
        [
            "cargo",
            "run",
            "--release",
            "--bin",
            "vis",
            "in/" + file_name,
            "out/" + file_name,
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
    )

    # ビジュアライズ結果をコピーして移動する処理
    # if os.path.exists("vis.html"):
    #     shutil.copy("vis.html", "result/visualizer/vis" + file_name + ".html")
    return res.stdout, res.stderr


# 入力がfile_nameであるテストケースを実行し、結果をresult.txtに出力してビジュアライズ結果を保存する
def run_test_case(file_name):
    run_time = exec_cpp(file_name)
    res, res2 = output(file_name, run_time)
    # print(res2)
    print(file_name, "DONE")
    return (file_name[:4] + " " + res)[:-1]


def main():
    init()
    txt_files = get_txt_list()  # 入力ファイルリストの獲得
    with multiprocessing.Pool(processes=num_processes) as p:
        l = p.map(run_test_case, txt_files)
    l.sort(key=lambda x: int(x[:4]))
    total = 0
    for i in l:
        total += log1p(int(i[13:]))
    print(len(txt_files))
    total /= len(txt_files)
    with open("result/result_score.txt", "w") as f:
        f.write("\n".join(l))
        f.write("\n" + str(total))
    return


if __name__ == "__main__":
    main()
