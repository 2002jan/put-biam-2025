import os
import json
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt


class AlgorithmRunStats:
    def __init__(self, iterations: int, scores: list[int], final_score: int, evaluations: int, partial_evaluations: int):
        self.iterations = iterations
        self.scores = scores
        self.final_score = final_score
        self.evaluations = evaluations
        self.partial_evaluations = partial_evaluations

class AlgorithmStats:
    def __init__(self, runs: list[dict], best_run: dict, worst_run: dict, optimum: int, avg_runtime: int):
        self.runs = [AlgorithmRunStats(**run) for run in runs]
        self.best_run = AlgorithmRunStats(**best_run)
        self.worst_run = AlgorithmRunStats(**worst_run)
        self.optimum = optimum
        self.avg_runtime = avg_runtime






def read_json(data_dir):

    data = {}
    for filename in os.listdir(data_dir):
        if filename.endswith(".json"):
            algorithm_name = filename.replace("_output.json", "")
            with open(os.path.join(data_dir, filename), "r") as f:
                data[algorithm_name] = AlgorithmStats(**json.load(f))

    return data


if __name__ == "__main__":
    path = "../outputs/2025_04_01_09_04_29/"
    data = read_json(path)

    # df_list = [extract_stats(name, algo_data) for name, algo_data in data.items()]
    # df = pd.concat(df_list, ignore_index=True)
    # print(df)

    # plt.figure(figsize=(10, 5))
    # sns.boxplot(x="Algorithm", y="Final Score", data=df)
    # plt.title("Final Score Distribution by Algorithm")
    # plt.show()


