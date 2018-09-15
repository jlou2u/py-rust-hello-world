import numpy as np
import pandas as pd
import libhello_world

a100 = np.arange(100)
a5 = np.array([5., 6., 7., 8., 9.])
df = pd.util.testing.makeTimeDataFrame()

print(libhello_world.hello_numpy(a100))

a5[0] = np.nan
print(libhello_world.hello_numpy(a5))

print(libhello_world.hello_numpy2d(df.values))
