// test de sécurité pour éviter l'inclusion multiple des définitions
#ifndef _CMESURE_H_
#define _CMESURE_H_

#include <iostream>
using namespace std;

#ifdef _CMESURE_EXPORTING
	#define CMESURE_DECLSPEC __declspec(dllexport)
#else
	#define CMESURE_DECLSPEC __declspec(dllimport)
#endif

///////////// Classe de gestion simultanée d'une mesure et de son incertitude ///////////

// Dans la suite, tous les calculs sont efectués en considérant les données indépendantes
// Il n'y a donc pas de gestion des covariances ... et allez vous faire foutre.


class CMESURE_DECLSPEC CMesure
{
private:

	double valeur;      // Valeur de la mesure
    double epsilon;     // Valeur de l'incertitude type (écart type de la variable aléatoire)
    double alpha;       // taux de confiance pour le calcul du coeff d'élargissement
                        // => par défaut à 95.45% équivalent à un coeff d'élargissement
						// de 2.00 en loi normale

public:

//////////////////////////// Constructeurs et destructeur ////////////////////////////

	// constructeurs
    CMesure();							   // valeur = 0.0; epsilon = CMESURE_EPS;	alpha = 95.45;
    CMesure(double v);                     // valeur = v;   epsilon = CMESURE_EPS;	alpha = 95.45;
    CMesure(double v, double e, double a); // valeur = v;   epsilon = fabs(e);		alpha = fabs(a);
	CMesure(char* m);					   // pour passer en paramètre la chaine ( valeur +/- IT | alpha% )

	CMesure(double v, double it, char loi = 'N');
	// Dans le cadre de mesures effectuées dans des conditions bien identifiées,
	// il est possible d'estimer l'incertitude type (epsilon) directement à partir
	// de l'intervalle de tolérance (it) à l'aide des lois suivante
	//
	//		1) 'R' : Résolution d'un indicateur numérique     : epsilon = it / rac(12.0)
	//		2) 'H' : Hystérésis tel que it = MAX - MIN        : epsilon = it / rac(12.0)
	//		3) 'S' : évolution Sinusoïdale sur it = MAX - MIN : epsilon = it / 1.4
	//		4) 'N' : loi Normale par défaut, K = 2            : epsilon = it / 2.0
	//		5) 'C' : appareil de Classe +/- it                : epsilon = it / rac(3.0)
	//
	// REM : la loi N est la loi par défaut dans tout bon certificat d'étalonnage qui se respecte

	// constructeur de recopie
	CMesure(const CMesure& X); 

	// destructeur
    ~CMesure();                             



////////////////// Accesseurs aux données privées de la classe /////////////////

	// Read only
	double Val(void);	// LA mesure en cours de traitement
    double Alpha(void);	// Taux de confiance
	double Eps(void);	// Incertitude type.
	double IT(void);	// Intervalle de tolérance = Eps x K
    double K(void);		// Coeff d'élargissement


    
    // surdéfinition des opérateurs
    
    CMesure  operator+ (const CMesure& M) const;
    CMesure& operator+=(const CMesure& M);

	CMesure  operator+ (const double&  V) const;
	CMesure& operator+=(const double&  V);

    CMesure  operator- (const CMesure& M) const;
    CMesure& operator-=(const CMesure& M);

	CMesure  operator- (const double&  V) const;
	CMesure& operator-=(const double&  V);

    CMesure  operator* (const CMesure& M) const;
    CMesure& operator*=(const CMesure& M);

	CMesure  operator* (const double&  V) const;
	CMesure& operator*=(const double&  V);

    CMesure  operator/ (const CMesure& M) const;
    CMesure& operator/=(const CMesure& M);

	CMesure  operator/ (const double&  V) const;
	CMesure& operator/=(const double&  V);

    CMesure& operator= (const CMesure& M);
	CMesure& operator= (const double&  V);

    bool operator==(const CMesure& M) const;
    bool operator!=(const CMesure& M) const;
    bool operator<=(const CMesure& M) const;
    bool operator>=(const CMesure& M) const;
    bool operator< (const CMesure& M) const;
    bool operator> (const CMesure& M) const;


////////////////////////// Fonctions amies de la classe /////////////////////////

	friend CMESURE_DECLSPEC CMesure operator-(CMesure& M);
	friend CMESURE_DECLSPEC CMesure operator+(double V, const CMesure& M);
	friend CMESURE_DECLSPEC CMesure operator-(double V, const CMesure& M);
	friend CMESURE_DECLSPEC CMesure operator*(double V, const CMesure& M);
	friend CMESURE_DECLSPEC CMesure operator/(double V, const CMesure& M);

    // affichage au format (valeur +/- IG)
    friend CMESURE_DECLSPEC ostream& operator<<(ostream& s, CMesure& M);
	
	// fonctions mathématique perso
	friend CMESURE_DECLSPEC CMesure Conjug(CMesure& M);
};

#endif	// fin de _CMESURE_H_
