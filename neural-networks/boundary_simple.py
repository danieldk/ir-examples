import matplotlib.pyplot as plt
import numpy as np

# From https://github.com/dennybritz/nn-from-scratch
#
# Modified for dynet by Daniël de Kok
def plot_decision_boundary(f,X,Y):
    padding=0.15
    res=0.01
    
    #max and min values of x and y of the dataset
    x_min,x_max=X[:,0].min(), X[:,0].max()
    y_min,y_max=X[:,1].min(), X[:,1].max()
    
    #range of x's and y's
    x_range=x_max-x_min
    y_range=y_max-y_min
    
    #add padding to the ranges
    x_min -= x_range * padding
    y_min -= y_range * padding
    x_max += x_range * padding
    y_max += y_range * padding

    #create a meshgrid of points with the above ranges
    xx,yy=np.meshgrid(np.arange(x_min,x_max,res),np.arange(y_min,y_max,res))
    
    #use model to predict class at each point on the grid
    #ravel turns the 2d arrays into vectors
    #c_ concatenates the vectors to create one long vector on which to perform prediction
    #finally the vector of prediction is reshaped to the original data shape.
    
    Z = []
    for (x_i, y_i) in zip(xx.ravel(), yy.ravel()):
        Z.append(1 if f(np.array([x_i, y_i])) > 0.5 else 0)
    
    Z = np.array(Z)
    Z = Z.reshape(xx.shape)
    
    #plot the contours on the grid
    plt.figure(figsize=(8,6))
    cs = plt.contourf(xx, yy, Z, cmap=plt.cm.Spectral)
    
    #plot the original data and labels
    plt.scatter(X[:,0], X[:,1], s=35, c=Y, cmap=plt.cm.Spectral)
