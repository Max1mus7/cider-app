import os
import glob
import pandas as pd

dirname = os.path.dirname(__file__)
filename = os.path.join(dirname, "../../metrics/win")
os.chdir(filename)

extension = 'csv'
all_filenames = [i for i in glob.glob('*.{}'.format(extension))]

#combine all files in the list
combined_csv = pd.concat([pd.read_csv(f) for f in all_filenames ])

#export to csv
combined_csv.to_csv("../combined_reports/combined.csv", index=False, lineterminator="\n")